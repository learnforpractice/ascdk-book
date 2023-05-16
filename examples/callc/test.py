import os
from ipyeos import eos
from ipyeos import chaintester
from ipyeos.chaintester import ChainTester
from ipyeos import log
from pyeoskit import eosapi

chaintester.chain_config['contracts_console'] = True
eos.set_log_level("default", 3)

logger = log.get_logger(__name__)

dir_name = os.path.dirname(os.path.abspath(__file__))

def init_test(contract_name):
    t = ChainTester(True)
    wasm_file = os.path.join(dir_name, f'{contract_name}.wasm')
    with open(wasm_file, 'rb') as f:
        code = f.read()

    abi_file = os.path.join(dir_name, f'{contract_name}.abi')
    with open(abi_file, 'r') as f:
        abi = f.read()

    t.deploy_contract('hello', code, abi)
    t.produce_block()
    eos.set_log_level("default", 1)
    return t

def test_say_hello():
    t = init_test('test')
    ret = t.push_action('hello', 'sayhello', "", {'hello': 'active'})
    t.produce_block()
    logger.info("++++++++++%s\n", ret['elapsed'])

