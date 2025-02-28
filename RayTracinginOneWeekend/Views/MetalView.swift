import Algorithm
import UIKit

class MetalView: UIView {
  private var device: MTLDevice! = MTLCreateSystemDefaultDevice()
  
  override class var layerClass: AnyClass {
    CAMetalLayer.self
  }
  
  required init?(coder: NSCoder) {
    super.init(coder: coder)
    setup()
  }
  
  override var frame: CGRect {
    didSet {
      let bytes = generateBMP(UInt(frame.width), UInt(frame.height))
      imageView.image = UIImage(data: Data(bytes))
    }
  }
  
  private lazy var imageView = UIImageView()
}

extension MetalView {
  private func setup() {
    addSubview(imageView)
    imageView.translatesAutoresizingMaskIntoConstraints = false
    NSLayoutConstraint.activate([
      imageView.topAnchor.constraint(equalTo: topAnchor),
      imageView.trailingAnchor.constraint(equalTo: trailingAnchor),
      imageView.bottomAnchor.constraint(equalTo: bottomAnchor),
      imageView.leadingAnchor.constraint(equalTo: leadingAnchor),
    ])
  }
}
