def generate_signal(model_output):
    # Mapping
    if model_output == "buy":
        return "Execute Buy Order"
    elif model_output == "sell":
        return "Execute Sell Order"
    else:
        return "Hold"
