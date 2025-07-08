const fs = require("fs");
const decoder = new TextDecoder();
const topic = "broadcast";
let text = "";

require("uWebSockets.js")
	.App()
	.ws("/*", {
		idleTimeout: 300,
		open: (ws) => {
			ws.subscribe(topic);
			ws.send(text);
		},
		message: (ws, message, isBinary) => {
			if (!isBinary) {
				text = decoder.decode(message);
				ws.publish(topic, text);
			}
		}
	})
	.get("/*", (res, req) => {
		let path = req.getUrl();
		let ret;

		if (path === "/") path = "/index.html";

		try {
			ret = fs.readFileSync(`./public${path}`, "utf-8")
		} catch {
			ret = "Error.";
		}
		res.writeStatus("200 OK").end(ret);
	})
	.listen(80, (listenSocket) => {
		if (listenSocket) {
			console.log("Listening to port 80");
		}
	});
