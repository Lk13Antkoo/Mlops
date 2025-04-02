#Using SQL CLI
'''bash
 ~/spark-3.4.1-bin-hadoop3/bin/spark-sql \
  --packages io.delta:delta-core_2.12:2.4.0 \
  --conf "spark.sql.extensions=io.delta.sql.DeltaSparkSessionExtension" \
  --conf "spark.sql.catalog.spark_catalog=org.apache.spark.sql.delta.catalog.DeltaCatalog"

'''

#Create a table in database
'''bash
CREATE DATABASES mydatabase;
'''


#Creat a table -> need to transform to temp view first
 '''bash
CREATE OR REPLACE TEMP VIEW temp_people
                      USING csv
                      OPTIONS (
                      path "/home/project/AZURE_DUKE/course_4/module_5_lab_1/Thomann_data_3.csv",
                      header "true",
                      inferSchema "true");
SELECT * FROM temp_people;

#====Create a table in the database
CREATE TABLE IF NOT EXISTS thomas_data
USING DELTA
AS SELECT * FROM temp_people;
'''


