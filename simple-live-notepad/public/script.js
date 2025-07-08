const main = document.getElementById("main");
const connection = new WebSocket(`ws://${window.location.host}/`);
connection.onmessage = (message) => {
	main.value = message.data;
};
connection.onclose = connection.onerror = () => (main.value = "[ERROR] Connection refused");
main.oninput = function () {
	connection.send(this.value);
};
