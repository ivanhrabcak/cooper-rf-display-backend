import os

from util import Util

from configparser import ConfigParser

class Config:
    @staticmethod
    def parse_config(path: str = "./config.conf") -> dict:
        if not os.path.exists(path):
            raise FileNotFoundError(f"The file '{path}' does not exist")

        config = ConfigParser.read_file(open(path))

        config_structrue = [
            ("edupage", dict),
            ("serial_port", str)
        ]

        edupage_config_structure = [
            ("username", str),
            ("password", str),
            ("subdomain", str)
        ]

        Util.ensure_all_fields(config, config_structrue)
        Util.ensure_all_fields(config["edupage"], edupage_config_structure)

        return config