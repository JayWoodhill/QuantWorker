def validate_trade_condition_code(code: str) -> bool:
    valid_codes = ["XT", "SH"]
    return code in valid_codes
