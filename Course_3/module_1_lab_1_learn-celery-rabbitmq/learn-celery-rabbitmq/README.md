# Distributed queuing with Flask, Celery, and RabbitMQ

Please install requirements

This repository will guide you through examples that you can use to implement
powerful queue patterns using RabbitMQ as the message broker.


## System requirements
Running with Linux

	Bước 1: chạy Celery app: bạn nên  vô path app luôn bởi vì bạn đang import main trong đó, các biến môi trường có thể ko hiểu path đó đâu nếu bạn ko trỏ thẳng nơi chứa main.py =))

	Bước 2: chạy Flask app bạn nên  vô path app luôn bởi vì bạn đang import main trong đó, các biến môi trường có thể ko hiểu path đó đâu nếu bạn ko trỏ thẳng nơi chứa main.py =)) rồi lệnh chạy app cho đỡ rắc rối.
	
	Bước 3: nhập lệnh:
	

 
![image](https://github.com/user-attachments/assets/600c35ab-a08e-4338-b529-745bbb6573b2)



## Start Celery

Celery needs to get started. First, make sure RabbitMQ is running by opening a terminal and running:

```
ps aux | grep rabbitmq
```

You should get output that displays the RabbitMQ process.

Start Celery by doing:

```bash
celery -A make_celery worker --loglevel INFO
```

You should see in the output that the `async_send_email` task is registered:

```
[tasks]
  . main.async_parse_exploits
  . main.async_send_email
```

## Start Flask

Start the Flask application with the following command:

```bash
flask --app main:flask_app run --reload
```

This will load the Flask application on port 5000 by default. Verify it is running by opening a browser and going to [http://localhost:5000](http://localhost:5000).

Try a different port if you are running into a port conflict:

```bash
flask --app main:flask_app run --reload -p 5001
```

## Send a request for async processing

Create a new `POST` request to the running application to the `/send_email` endpoint. You can use the following `curl` command:

```bash
curl -X POST --header "Content-Type: application/json" --data '{"email": "john.doe@example.org", "subject": "hi from Celery!", "body": "Just a test"}' http://localhost:5000/send_email
```

This request will not return any values and it mimics a "fire and forget" type of processing. The request will be sent to the Flask application, which will then send the request to Celery for processing. Celery will then send the request to RabbitMQ for processing. RabbitMQ will then send the request to a Celery worker for processing. The Celery worker will then process the request and send the email.

## Send a request for async processing with a response

Create a new `POST` request to the running application to the `/parse_exploits` endpoint. You can use the following `curl` command:

```bash
curl -X POST http://localhost:5000/parse_exploits
```

You will get a response that looks like the following:

```json
{
    "task_id": 1
}
```

You can then use the `task_id` to query the status of the task to see if it is complete. You can use the following `curl` command that will request the information to the endpoint `/check_task/<task_id>`:

```bash
curl -X GET http://localhost:5000/check_task/1
```

You will get a response that looks like the following:

```json
{
        "task_id": 1, 
        "task_status": "PENDING", 
        "task_result": [[CVE-2019-1622, CVE-2019-1623, CVE-2019-1624, CVE-2019-1625, CVE-2019-1626, CVE-2019-1627, CVE-2019-1628, CVE-2019-1629, CVE-2019-1630, CVE-2019-1631, CVE-2019-1632, CVE]]
}
```
