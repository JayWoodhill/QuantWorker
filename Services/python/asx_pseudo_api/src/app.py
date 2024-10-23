from fastapi import FastAPI, Request, HTTPException
from pydantic import BaseModel
from typing import List
import datetime
from .session import SessionManager
from .config import allowed_ips

app = FastAPI()

session_manager = SessionManager()

# Pydantic models for trade capture report
class TradeSide(BaseModel):
    side: int        # 1 for buy, 2 for sell
    quantity: int
    price: float

class TradeCaptureReport(BaseModel):
    trade_date: datetime.date
    sides: List[TradeSide]
    trd_condition_code: str
    last_msg_seq_num_processed: int

# Check IP whitelisting
@app.middleware("http")
async def check_ip_whitelisting(request: Request, call_next):
    client_ip = request.client.host
    if client_ip not in allowed_ips:
        raise HTTPException(status_code=403, detail="IP not allowed")
    response = await call_next(request)
    return response

@app.post("/trade")
async def capture_trade(trade: TradeCaptureReport):
    # Check that trade_date is today
    if trade.trade_date != datetime.date.today():
        raise HTTPException(status_code=400, detail="Trade date must be today")

    # Process trade sides
    for side in trade.sides:
        if side.side not in (1, 2):
            raise HTTPException(status_code=400, detail="Invalid side")
    
    # Validate message sequencing
    if not session_manager.validate_sequence(trade.last_msg_seq_num_processed):
        raise HTTPException(status_code=400, detail="Invalid sequence number")

    # Simulate processing the trade capture report
    return {"message": "Trade capture report processed successfully", "sequence": trade.last_msg_seq_num_processed}

@app.get("/health")
async def health_check():
    return {"status": "API is running"}
