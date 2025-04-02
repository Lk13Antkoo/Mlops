# This course is about AZURE service

## 1. Describe

**api_temperature**:test out a microservice architecture, no model usage, use FastAPI() to mimic a web application.

**azure-chat-demo**: invoke the Azure model using multiple functions, check the examples for details (simple chat, using system prompt, using plugin functino, using advanced function, native function and micro service).

**azure-rag**: refer example to use Azure cognitive Search ( AzureSearch ) to upoad data to AzureSearch, invoke model. After uploading the document to AzureSearch, refer webapp folder to itegrate: Invoke Azure model + Search + FastAPI.

## 2. Check the path
```
.
├── README.md
├── api_temperature
│   └── historical-temperatures
├── azure-chat-demo
│   ├── LICENSE
│   ├── README.md
│   ├── chat.py
│   ├── examples
│   └── requirements.txt
└── azure-rag
    ├── Dockerfile
    ├── LICENSE
    ├── README.md
    ├── examples
    ├── requirements.txt
    ├── webapp
    └── wine-ratings.csv
```