import logging
import sys
import tempfile

import time
from pprint import pprint
import json
import pendulum
import ast
from airflow import DAG
from airflow.decorators import task
from airflow.operators.python import PythonVirtualenvOperator, is_venv_installed

log = logging.getLogger(__name__)

PATH_TO_PYTHN_BINARY = sys.executable



def download_census_data():
    import pandas as pd
    import numpy as np
    url = "https://raw.githubusercontent.com/practical-bootcamp/week4-assignment1-template/main/city_census.csv"
    df = pd.read_csv(url,index_col=0)
    df = df.fillna("LINHDAN")
    df = df.reset_index()
    #df.replace(to_replace=pd.NA, value="Nonono", inplace=True)
    return df.head(10).to_dict()

def checking_data(data):
    import pandas as pd
    import numpy as np
    import json
    """
    Lỗi tè le, nê dùng PythonOperator:

    https://airflow.apache.org/docs/apache-airflow/stable/howto/operator/python.html#pythonvirtualenvoperator

    task_instance = kwargs['task_instance']
    temp_data = task_instance.xcom_pull(task_ids="download_census_data")
    """

    temp_data = data
    print(type(temp_data))
    temp_data = temp_data.replace("'", "\"")
    temp_data = json.loads(temp_data)
    df = pd.DataFrame(temp_data)    
    return df.info()


with DAG(
    dag_id = "census_data_pipeline",
    start_date = pendulum.datetime(2023, 11, 25, tz="UTC"), 
    schedule = None,
    tags = ['create_pipeline_lab_2']
) as dag:
    if not is_venv_installed():
        log.warning("There is no Virtual Env, pls, install")
    else:
        
        """
        @task.virtualenv(
        task_id= "create_virtual_env", requirements= ['pandas==2.1.1', 'numpy>=1.21,<1.25'], system_site_packages = False
    )
        def get_path_virtualenv():
            pass
        """

        

        running_data_process_1 = PythonVirtualenvOperator(
            task_id = "download_census_data",
            python_callable=download_census_data,
            #python_env="{{ ti.xcom_pull(task_ids='pass_virtualenv_path') }}"
            requirements=["pandas==2.1.1", "numpy>=1.21,<1.25"],
            system_site_packages=False,
            #============ Dùng khi bạn muốn đưa context vào hàm
            #provide_context=True   
        )


        running_data_process_2 = PythonVirtualenvOperator(
            task_id = "checking_data",
            python_callable = checking_data,
            op_kwargs={'data': '{{ ti.xcom_pull(task_ids="download_census_data") }}'}, 
            #python_env="{{ ti.xcom_pull(task_ids='pass_virtualenv_path') }}" 
            requirements=["pandas==2.1.1", "numpy>=1.21,<1.25"],
            #====== dùng True khi bạn muốn import library từ Python env chính
            system_site_packages=False,
            #provide_context=True
        )   
        
       # task_0 =  get_path_virtualenv()
        #task_1 = running_data_process_1
        #task_2 = running_data_process_2
        running_data_process_1 >> running_data_process_2
        #task_0 >> [task_1, task_2]