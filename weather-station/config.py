import os

from .util import Util
from typing import Union

from configparser import ConfigParser

class Config:
    @staticmethod
    def parse_config(path: str = "./config.conf") -> dict:
        if not os.path.exists(path):
            raise FileNotFoundError(f"The file '{path}' does not exist")

        config = ConfigParser()
        config.read_file(open(path))

        config_structrue = [
            ("edupage", dict),
            ("serial", dict)
        ]

        edupage_config_structure = [
            ("username", str),
            ("password", str),
            ("subdomain", str)
        ]

        serial_port_structure = [
            ("serial_port", str)
            ("data_path", str)
        ]

        netatmo_config_structure = [
            ("devices", Union[str, list]),
            ("client_secret", str),
            ("refresh_token", str)
        ]

        config = config._sections

        Util.ensure_all_fields(config, config_structrue)
        Util.ensure_all_fields(config["edupage"], edupage_config_structure)
        Util.ensure_all_fields(config["serial"], serial_port_structure)
        Util.ensure_all_fields(config["netatmo"], netatmo_config_structure)

        return {
            "data_path": config["serial"][data_directory],
            "serial_port": config["serial"]["serial_port"],
            "edupage": config["edupage"],
            "netatmo": config["netatmo"]
        }