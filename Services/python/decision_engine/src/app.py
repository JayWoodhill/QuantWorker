import zmq
import asyncio
from fastapi import FastAPI
# data ingestion app
app = FastAPI()

context = zmq.Context()
socket = context.socket(zmq.SUB)
socket.connect("tcp://asx_pseudo_api:5555")  # Replace with actual address
socket.setsockopt_string(zmq.SUBSCRIBE, "")  # Subscribe to all messages

async def listen_to_market_data():
    while True:
        message = socket.recv_json()
        process_trade_data(message)  # Process trade data through models

def process_trade_data(trade_data):
    # Process the trade data using the models
    decision = apply_trading_model(trade_data)
    return decision

@app.on_event("startup")
async def startup_event():
    asyncio.create_task(listen_to_market_data())
