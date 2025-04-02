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
    dag_id = "trigger_DAG",
    schedule= [PATH_SAVE_CSV], #trigger when this change
    start_date=pendulum.datetime(2023, 11, 25, tz="UTC"),  # Add a start_date
    tags = ['example_1']
) as dag:
    if not is_venv_installed():
        log.warning("There is no virtual enviromnent for Python, pls install")
    
    else:
        @task.virtualenv(
            task_id = 'python_virtualenv', requirements = ['pandas==2.1.1', 'numpy>=1.21,<1.25'], system_site_packages = False  
        )
        def pandas_write():
            import pandas as pd
            df = pd.read_csv("/home/project/AZURE_DUKE/course_3/module_2/lab_2_Create_dag_pandas/wine_download.csv", index_col=0)
            head = df.head(5)
            return head.to_csv("/home/project/AZURE_DUKE/course_3/module_2/lab_2_Create_dag_pandas/wine_fine_funed.csv")
        
        pandas_task = pandas_write()