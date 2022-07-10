import React, { useState } from 'react'
import { invoke, process } from '@tauri-apps/api'
function App() {
  const [text, setText] = useState('')

  const addTask = async (e: React.KeyboardEvent) => {
    switch (e.key) {
      // при нажатии `Enter` вызываем `add_task` с текстом заметки
      case 'Enter':
        try {
          await invoke('add_task', { text })
          setText('')
        } catch (e) {
          console.error(e)
        }
        break
      // при нажатии `Esc` завершаем процесс
      case 'Escape':
        return process.exit()
      default:
        return
    }
  }

  return (
    <div className='divMain'>
      <p>Black Box</p>
      <button>Start</button>
        <input type={"checkbox"}></input>
        <p>Save profile</p>
      <div className='inputs'>
        <input type={"text"} placeholder="URL to server"></input>
        <input type={"text"} placeholder="Your secret token"></input>
        <input type={"text"} placeholder="Login"></input>
        <input type={"text"} placeholder="Password"></input>
      </div>
      <p>Vileos for loading:</p>
    </div>
  )
}



export default App