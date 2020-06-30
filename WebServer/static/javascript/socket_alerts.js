function create_alert(alert_type, title, content) {
    div = document.createElement('div');
    div.classList.add('alert');
    div.classList.add('alert-' + alert_type);
    div.classList.add('alert-dismissible');
    div.classList.add('fade');
    div.classList.add('show');
    div.setAttribute('role', 'alert');
    div.innerHTML = `
    <strong>${title}</strong> ${content}
    <button type="button" class="close" data-dismiss="alert" aria-label="Close">
        <span aria-hidden="true">&times;</span>
    </button>`;
    document.getElementById('notifications').appendChild(div);
}

$(document).ready(function() {
    let socket = new WebSocket("ws://192.168.0.26:3012/socket");

    socket.onopen = function(e) {
        create_alert("success", "Connection established", "Sending 'i'm here'");
        socket.send("i'm here");
    };

    socket.onmessage = function(event) {
        create_alert("primary", "New message", `Data received from server: ${event.data}`);
    };

    socket.onclose = function(event) {
        if (event.wasClean) {
            create_alert("warning", "Socket closed", `Connection closed cleanly, code=${event.code} reason=${event.reason}`);
        } else {
            // e.g. server process killed or network down
            // event.code is usually 1006 in this case
            create_alert("warning", "Socket closed", `Connection died`);
        }
    };

    socket.onerror = function(error) {
        create_alert("danger", "Error", `${error.message}`);
    };
});