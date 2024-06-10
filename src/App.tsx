import Inputfield from "./GUI/main/Inputfield";

import { invoke } from '@tauri-apps/api/tauri'

async function get_book() {
  // const book = document.querySelector('#book')!;
  // book.textContent = await invoke('get_book', { code: 'examplecode'});
  // const renderedText = htmlElement.innerText;
  console.log(await invoke('get_book', { code: 'examplecode'}));
}

function App() {
  return (
    <div className="text-black h-[100%] w-[100%]">
      <Inputfield />
      <p id="book"></p>
    </div>
  );
}

export default App;
// westermann.de/webcodes/webcode={code}
// codes.cornelsen.de/codes/webcode-ws/api/v1/searchAll?searchterm={code}