""""
This lab create a local web service than get access to local file and upload it into http
The file is created in procedure from README.md
"""

import csv
from http.server import HTTPServer, BaseHTTPRequestHandler

class HTTPHandler(BaseHTTPRequestHandler):

    def do_GET(self):
        self.send_response(200)
        self.end_headers()
        with open('/var/lib/mysql-files/test_1_actor.csv') as f:
            reader =  csv.reader(f)
            self.wfile.write(b"\n".join([','.join(row).encode() for row in reader]))

httpd = HTTPServer(('localhost',8080), HTTPHandler)
httpd.serve_forever()
