import React, { useState } from 'react'
import { invoke, process } from '@tauri-apps/api'
import Imputs from './Components/Imputs'

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
      <div className='inputs'>
     <Imputs />
      </div>
      <button>Start</button>
      <p>Vileos for loading:</p>
    </div>
  )
}



export default App