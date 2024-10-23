def apply_trading_model(trade_data):
    # Sample model
    buy_price = trade_data['buy_price']
    sell_price = trade_data['sell_price']
    if buy_price < sell_price:
        return "buy"
    elif sell_price > buy_price:
        return "sell"
    else:
        return "hold"
