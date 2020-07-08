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

function create_socket_connection(ip, port, request) {
    let socket = new WebSocket(`ws://${ip}:${port}/${request}`);

    socket.onopen = function(e) {
        create_alert("success", "Connection established", "New messages will be shown here.");
        socket.send(request);
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

    return socket;
}