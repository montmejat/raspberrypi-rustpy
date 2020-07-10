function custom_on_message_call(data) {
    // To define in HTML page
}

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

    socket.onmessage = function(event) {
        try {
            data = JSON.parse(event.data);
            
            if (data.navbar.action != "") {
                document.getElementById('action_link').onclick = function() {
                    send_command(socket, data.navbar.action);
                    if (data.navbar.action == "pause") {
                        document.getElementById("icon_name").src = `bootstrap-icons-1.0.0-alpha4/play.svg`;
                    } else {
                        document.getElementById("icon_name").src = `bootstrap-icons-1.0.0-alpha4/pause.svg`;
                    }
                };
                
                document.getElementById("action_link").href = "#";
            } else {
                document.getElementById("action_link").removeAttribute("href");
            }
            
            custom_on_message_call(event.data);
        } catch(e) {
            create_alert("primary", "New message", `${event.data}`);
        }
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

function send_command(socket, command) {
    socket.send(JSON.stringify({ "command": command }));
}