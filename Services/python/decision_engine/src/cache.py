import redis

cache = redis.Redis(host='localhost', port=6379, db=0)

def cache_trade_data(trade_data):
    cache.set(f"trade:{trade_data['id']}", trade_data)

def get_cached_trade(trade_id):
    return cache.get(f"trade:{trade_id}")
