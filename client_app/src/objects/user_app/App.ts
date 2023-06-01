// user_app_common objects types
import init, {process_main_response, MainResponse, WasmMainResponse} from 'user_wasm_lib'
let isInit = false;

export default async function handle_response(r: MainResponse) {
  if(!isInit) {
    isInit = true;
    await init();
  }
  if (r) {
    return process_main_response(r) as WasmMainResponse;
  }
} 