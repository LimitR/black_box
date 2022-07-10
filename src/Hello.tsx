import React, { useState } from 'react'
import { invoke, process } from '@tauri-apps/api'
function Hello() {

  const hello = async () => {
    await invoke('hello_line');
  }

  return (
    <button
      type='button'
      className='btn'
      onClick={() => hello()}>
        Hello
    </button>
  )
}



export default Hello