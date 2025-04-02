from qdrant_client import QdrantClient

# Connect to Qdrant (local or remote)
client = QdrantClient(host="localhost", port=6333)

search_result = client.query_points(
    collection_name="test_collection",
    query=[0.2, 0.1, 0.9, 0.7],
    with_payload=False,
    limit=3
).points

print(search_result)
