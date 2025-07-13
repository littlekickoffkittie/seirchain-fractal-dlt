import UIKit

class ViewController: UIViewController {

    @IBOutlet weak var userIdTextField: UITextField!
    @IBOutlet weak var walletAddressLabel: UILabel!

    override func viewDidLoad() {
        super.viewDidLoad()
        // Do any additional setup after loading the view.
    }

    @IBAction func createWallet(_ sender: Any) {
        guard let userId = userIdTextField.text, !userId.isEmpty else {
            return
        }

        let url = URL(string: "http://127.0.0.1:8080/api/create_wallet")!
        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")

        let body = ["user_id": userId]
        let bodyData = try? JSONSerialization.data(withJSONObject: body)

        request.httpBody = bodyData

        let task = URLSession.shared.dataTask(with: request) { data, response, error in
            guard let data = data, error == nil else {
                return
            }

            let responseJSON = try? JSONSerialization.jsonObject(with: data, options: [])
            if let responseJSON = responseJSON as? [String: Any], let address = responseJSON["address"] as? String {
                DispatchQueue.main.async {
                    self.walletAddressLabel.text = "Wallet Address: \(address)"
                }
            }
        }

        task.resume()
    }

}
