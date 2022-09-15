from dataclasses import dataclass
from .reading import parse_reading
import serial


@dataclass
class Dongle:
    port: str
    is_initialized: bool = False

    def init(self):
        self.serial_port = serial.Serial(self.port, 115200, timeout=3600)
        self.is_initialized = True
    
    def close(self):
        self.serial_port.close()
    
    def read_until_terminator(self) -> str:
        output = self.serial_port.read_until(b"\r\n").decode()

        if "ERROR" in output:
            raise IOError("Error response from dongle!")

        return output.replace("\r\n", "")

    def get_id(self) -> str:
        self.serial_port.write(b"AT+CGSN\r\n")

        id = self.read_until_terminator()
        self.read_until_terminator()

        return id.split("+CGSN: ")[1]
    
    
    def get_stations(self) -> dict[str, str]:
        """Returns a dictionary:
            ```python3
            {
                "station_id": "station_name",
                "station1_id": "station1_name,
                # ...
            }
        """

        self.serial_port.write(b"AT$LIST\r\n")

        station_identifiers = {}

        # in this format: `{station_id},"{station_name}"`
        identifier = self.read_until_terminator()
        
        while identifier != "OK":
            print(identifier)
            if "$" in identifier:
                identifier = self.read_until_terminator()
                continue

            id, name = identifier.split(",")
            name = name.replace('"', "")

            station_identifiers[id] = name
            
            identifier = self.read_until_terminator()
        
        return station_identifiers
    
    def wait_for_reading(self) -> dict:
        while True:
            reading = self.read_until_terminator()
            if "$RECV" not in reading:
                continue
            
            return parse_reading(reading.split("$RECV: ")[1])

