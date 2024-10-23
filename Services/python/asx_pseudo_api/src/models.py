from pydantic import BaseModel
from typing import List
import datetime

# trading model input template

class TradeSide(BaseModel):
    side: int
    quantity: int
    price: float

class TradeCaptureReport(BaseModel):
    trade_date: datetime.date
    sides: List[TradeSide]
    trd_condition_code: str
    last_msg_seq_num_processed: int