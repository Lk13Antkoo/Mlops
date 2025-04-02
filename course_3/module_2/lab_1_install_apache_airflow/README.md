Start airflow: airflow standalone -> Serve for Lap_2
remember to active everytime you run again a virtual environment in Ubuntu

Follow these steps to install Apache Airflow:

1. Install dependencies: Ensure you have Python 3.7+ installed

2. Create a constraints file. Set the Airflow version and Python version

C3. onstruct the constraints URL

Install Airflow

1. Source the constraints file: source constraints.sh

2. Install via pip:

pip install "apache-airflow==${AIRFLOW_VERSION}" --constraint "${CONSTRAINT_URL}"

Initialize the database

Start Airflow components

Access the UI at 
http://localhost:8080