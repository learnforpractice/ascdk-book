import os
import sys
import json
import struct
import pytest

test_dir = os.path.dirname(__file__)
sys.path.append(os.path.join(test_dir, '..'))

from ipyeos import log
from ipyeos import chaintester
from ipyeos.chaintester import ChainTester

chaintester.chain_config['contracts_console'] = True

logger = log.get_logger(__name__)

def update_auth(tester, account):
    a = {
        "account": account,
        "permission": "active",
        "parent": "owner",
        "auth": {
            "threshold": 1,
            "keys": [
                {
                    "key": 'EOS6AjF6hvF7GSuSd4sCgfPKq5uWaXvGM2aQtEUCwmEHygQaqxBSV',
                    "weight": 1
                }
            ],
            "accounts": [{"permission":{"actor":account,"permission": 'eosio.code'}, "weight":1}],
            "waits": []
        }
    }
    tester.push_action('eosio', 'updateauth', a, {account:'active'})

def init_tester():
    tester = chaintester.ChainTester()
    update_auth(tester, 'hello')
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
def deploy_contract(tester):
    with open(f'{test_dir}/target/sender.wasm', 'rb') as f:
        code = f.read()
    with open(f'{test_dir}/target/sender.abi', 'rb') as f:
        abi = f.read()
    tester.deploy_contract('alice', code, abi)


    with open(f'{test_dir}/target/receiver.wasm', 'rb') as f:
        code = f.read()
    with open(f'{test_dir}/target/receiver.abi', 'rb') as f:
        abi = f.read()
    tester.deploy_contract('hello', code, abi)

@chain_test
def test_notify(tester):
    deploy_contract(tester)
    args = {'name': 'alice'}
    r = tester.push_action('alice', 'sayhello', args, {'alice': 'active'})
    logger.info('++++++elapsed: %s', r['elapsed'])
    tester.produce_block()
