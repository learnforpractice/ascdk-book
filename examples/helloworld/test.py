import os
import sys
import json
import struct
import pytest

test_dir = os.path.dirname(__file__)
sys.path.append(os.path.join(test_dir, '..'))

from ipyeos import log
from ipyeos import eos
from ipyeos import chaintester
from ipyeos.chaintester import ChainTester

chaintester.chain_config['contracts_console'] = True

logger = log.get_logger(__name__)

def init_tester():
    tester = chaintester.ChainTester()
    return tester

def chain_test(fn):
    def call():
        tester = init_tester()
        ret = fn(tester)
        tester.free()
        return ret
    return call

class NewChainTester():
    def __init__(self):
        self.tester = None

    def __enter__(self):
        self.tester = init_tester()
        return self.tester

    def __exit__(self, type, value, traceback):
        self.tester.free()

test_dir = os.path.dirname(__file__)
def deploy_contract(tester, package_name):
    with open(f'{test_dir}/target/{package_name}.wasm', 'rb') as f:
        code = f.read()
    with open(f'{test_dir}/target/{package_name}.abi', 'rb') as f:
        abi = f.read()
    tester.deploy_contract('hello', code, abi)

@chain_test
def test_sayhello(tester):
    deploy_contract(tester, 'helloworld')
    ret = tester.push_action('hello', 'sayhello', "", {'hello': 'active'})
    tester.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])
