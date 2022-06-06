//import { invoke } from '@tauri-apps/api/tauri'
//const {invoke} = reqiure("@tauri-apps/api/tauri")
const Invoke = window.__TAURI__.invoke
var path=1
document.querySelector("#SS").addEventListener("click",()=>
{
      //call rust backend function to take screen shot
       Invoke('Screen_shot',{ var:path.toString()})
       path = path+1;

       //window.alert to let user know that screen sho
       alert("you took Screen shot")

})


