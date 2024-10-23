class SessionManager:
    def __init__(self):
        self.current_sequence = 0

    def validate_sequence(self, incoming_seq: int) -> bool:
        if incoming_seq == self.current_sequence + 1:
            self.current_sequence += 1
            return True
        return False

    def reset_sequence(self):
        self.current_sequence = 0
