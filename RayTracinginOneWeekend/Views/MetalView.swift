import Algorithm
import UIKit

class MetalView: UIView {
  private lazy var device: MTLDevice! = MTLCreateSystemDefaultDevice()
  private lazy var metalLayer: CAMetalLayer! = layer as? CAMetalLayer
  private var commandQueue: MTLCommandQueue!
  private var vertexBuffer: MTLBuffer!
  private var texture: MTLTexture?
  private var pipelineState: MTLRenderPipelineState!

  override class var layerClass: AnyClass {
    CAMetalLayer.self
  }

  override var frame: CGRect {
    didSet {
      let bytes = generateRawData(UInt(bounds.width), UInt(bounds.height))
      updateTexture(from: Array(bytes), width: bounds.width, height: bounds.height)
    }
  }

  required init?(coder: NSCoder) {
    super.init(coder: coder)
    setup()
    setupPipeline()
    setupVertexBuffer()
  }
}

extension MetalView {
  private func setup() {
    metalLayer.device = device
    metalLayer.pixelFormat = .bgra8Unorm
    metalLayer.contentsScale = UIWindow.first?.screen.scale ?? 3
    commandQueue = device.makeCommandQueue()
  }

  private func setupPipeline() {
    let library = device.makeDefaultLibrary()
    let vertexFn = library?.makeFunction(name: "vertex_fn")
    let fragmentFn = library?.makeFunction(name: "fragment_fn")
    let pipelineDescriptor = MTLRenderPipelineDescriptor()
    pipelineDescriptor.vertexFunction = vertexFn
    pipelineDescriptor.fragmentFunction = fragmentFn
    pipelineDescriptor.colorAttachments[0].pixelFormat = metalLayer.pixelFormat
    do {
      pipelineState = try device.makeRenderPipelineState(descriptor: pipelineDescriptor)
    } catch {
      fatalError("Failed to create pipeline state: \(error)")
    }
  }

  private func setupVertexBuffer() {
    let vertices: [Float] = [
      -1, 1, 0, 1,
      1, 1, 1, 1,
      -1, -1, 0, 0,
      1, -1, 1, 0,
    ]
    vertexBuffer = device.makeBuffer(bytes: vertices, length: vertices.count * MemoryLayout<Float>.size, options: [])
  }

  func updateTexture(from rawData: [UInt8], width: CGFloat, height: CGFloat) {
    let descriptor = MTLTextureDescriptor()
    descriptor.pixelFormat = .rgba8Unorm
    descriptor.width = Int(width)
    descriptor.height = Int(height)
    descriptor.usage = [.shaderRead, .shaderWrite]
    texture = device.makeTexture(descriptor: descriptor)
    let bytesPerRow = Int(width) * 4
    rawData.withUnsafeBytes { rawBufferPointer in
      texture?.replace(
        region: MTLRegionMake2D(0, 0, Int(width), Int(height)),
        mipmapLevel: 0,
        withBytes: rawBufferPointer.baseAddress!,
        bytesPerRow: bytesPerRow
      )
    }
    drawFrame()
  }

  func drawFrame() {
    guard let drawable = metalLayer.nextDrawable(),
          let pipelineState,
          let commandBuffer = commandQueue.makeCommandBuffer(),
          let renderPassDescriptor = currentRenderPassDescriptor(for: drawable),
          let renderEncoder = commandBuffer.makeRenderCommandEncoder(descriptor: renderPassDescriptor),
          let vertexBuffer,
          let texture else { return }

    renderEncoder.setRenderPipelineState(pipelineState)
    renderEncoder.setVertexBuffer(vertexBuffer, offset: 0, index: 0)
    renderEncoder.setFragmentTexture(texture, index: 0)
    renderEncoder.drawPrimitives(type: .triangleStrip, vertexStart: 0, vertexCount: 4)

    renderEncoder.endEncoding()
    commandBuffer.present(drawable)
    commandBuffer.commit()
  }

  private func currentRenderPassDescriptor(for drawable: CAMetalDrawable) -> MTLRenderPassDescriptor? {
    let renderPassDescriptor = MTLRenderPassDescriptor()
    renderPassDescriptor.colorAttachments[0].texture = drawable.texture
    renderPassDescriptor.colorAttachments[0].loadAction = .clear
    renderPassDescriptor.colorAttachments[0].clearColor = MTLClearColor(red: 0, green: 0, blue: 0, alpha: 1)
    renderPassDescriptor.colorAttachments[0].storeAction = .store
    return renderPassDescriptor
  }
}
