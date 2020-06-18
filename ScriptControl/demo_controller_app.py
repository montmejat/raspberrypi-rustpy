import scriptcontrol as sc

if __name__ == '__main__':
    server_socket = sc.start_server(print_debug=True)
    sc.app_loop(server_socket, print_debug=True)