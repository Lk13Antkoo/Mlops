"""
This code will download csv file from web and save it to local
Please define: PATH_SAVE_CSV
schedule: None in DAG
outlets = [PATH_SAVE_CSV] in @task.virtualenv
"""

import logging
import sys
import tempfile

import time
from pprint import pprint

import pendulum

from airflow import DAG, Dataset
from airflow.decorators import task
from airflow.operators.python import PythonVirtualenvOperator, is_venv_installed

log = logging.getLogger(__name__)

PATH_TO_PYTHN_BINARY = sys.executable

PATH_SAVE_CSV = Dataset("file://wsl.localhost/Ubuntu/home/project/AZURE_DUKE/course_3/module_2/lab_2_Create_dag_pandas/wine_download.csv")

with DAG (
    dag_id = "creat_pandas_example",
    schedule= None,
    start_date=pendulum.datetime(2023, 11, 25, tz="UTC"),  # Add a start_date
    tags = ['example_1']
) as dag:
    if not is_venv_installed():
        log.warning("There is no virtual enviromnent for Python, pls install")
    
    else:
        @task.virtualenv(
            task_id = 'python_virtualenv', requirements = ['pandas==2.1.1', 'numpy>=1.21,<1.25'], system_site_packages = False,
            #=====outlets: tiện ích của airflow cho phép các DAG khác dựa vào nó để trigger khi outlets này thay đổi
            outlets=[PATH_SAVE_CSV]  
        )
        def pandas_head():
            import pandas as pd
            csv_url = "https://raw.githubusercontent.com/paiml/wine-ratings/main/wine-ratings.csv"
            df = pd.read_csv(csv_url, index_col=0)
            head = df.head(10)
            return head.to_csv("/home/project/AZURE_DUKE/course_3/module_2/lab_2_Create_dag_pandas/wine_download.csv")
        
        pandas_task = pandas_head()