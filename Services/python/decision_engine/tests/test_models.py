import unittest
from src.models import apply_trading_model

class TestTradingModels(unittest.TestCase):
    def test_apply_trading_model_buy(self):
        trade_data = {"buy_price": 100, "sell_price": 150}
        decision = apply_trading_model(trade_data)
        self.assertEqual(decision, "buy")

    def test_apply_trading_model_sell(self):
        trade_data = {"buy_price": 200, "sell_price": 150}
        decision = apply_trading_model(trade_data)
        self.assertEqual(decision, "sell")
