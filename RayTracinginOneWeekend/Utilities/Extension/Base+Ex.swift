import UIKit

extension UIWindow {
  static var first: UIWindow? {
    for scene in UIApplication.shared.connectedScenes {
      guard let windowScene = scene as? UIWindowScene else { continue }
      return windowScene.windows.first
    }
    return .none
  }
}
