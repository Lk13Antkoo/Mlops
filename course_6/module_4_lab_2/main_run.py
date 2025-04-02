from fastapi import FastAPI, Response
from pydantic import BaseModel
import pandas as pd
import os
from os.path import dirname
import openai
from qdrant_client import models, QdrantClient
from sentence_transformers import SentenceTransformer
from dotenv import load_dotenv

load_dotenv()

openai.api_base = os.getenv("OPENAI_API_BASE")
openai.api_key = os.getenv("OPENAI_API_KEY")
model_name = os.getenv("MODEL_NAME")

app = FastAPI()

#==== create document
df = pd.read_csv('top_rated_wines.csv')
df = df[df['variety'].notna()] # remove any NaN values as it blows up serialization
data = df.sample(700).to_dict('records') # Get only 700 records. More records will make it slower to index
len(data) 

#=== embedding
encoder = SentenceTransformer("all-MiniLM-L6-v2")

#==== qdrant
client = QdrantClient(":memory:")
client.create_collection(
    collection_name="my_wine",
    vectors_config=models.VectorParams(
        size=encoder.get_sentence_embedding_dimension(),  # Vector size is defined by used model
        distance=models.Distance.COSINE,
    ),
)

client.upload_points(
    collection_name="my_wine",
    points=[
        models.PointStruct(
            id=idx, vector=encoder.encode(doc["notes"]).tolist(), payload=doc
        )
        for idx, doc in enumerate(data)
    ],
)

def AI_chat(text):

    #=====query vector database
    hits = client.search(
    collection_name="my_wine",
    query_vector=encoder.encode("text").tolist(),
    limit=3,)

    augmented_context = [hit.payload for hit in hits]

    message_text = [
       {"role":"system","content":"You are a friendly AI assistant that helps people find information and answer questions."},
       {"role": "user", "content": text},
       {"role":"assistant", "content": str(augmented_context)},
    ]

    completion = openai.ChatCompletion.create(
      model=model_name,
      messages=message_text,
      temperature=0.7,
      max_tokens=800,
      top_p=0.95,
      frequency_penalty=0,
      presence_penalty=0,
      stop=None
    )
    return completion

class Body(BaseModel):
    text: str

@app.post('/generate')
def generate(body: Body):
    completion = AI_chat(body.text)
    return {"text": completion['choices'][0]['message']['content']}