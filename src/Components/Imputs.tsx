import React, { useState } from "react";
import { invoke, process } from '@tauri-apps/api'

const Imputs = () => {
    const [url, setUrl] = useState('')
    const [token, setToken] = useState('')
    const [login, setLogin] = useState('')
    const [password, setPassword] = useState('')
    const saveProfile = async () => {
        await invoke('save_profile', {
            data: JSON.stringify({
                url,
                token,
                login,
                password,
            })
        });
    }
    return (
        <div className="input">
            <input type={"text"} onChange={(e) => setUrl(e.target.value)} value={url} placeholder="URL to server"></input>
            <input type={"text"} onChange={(e) => setToken(e.target.value)} value={token} placeholder="Your secret token"></input>
            <input type={"text"} onChange={(e) => setLogin(e.target.value)} value={login} placeholder="Login"></input>
            <input type={"text"} onChange={(e) => setPassword(e.target.value)} value={password} placeholder="Password"></input>
            <input type={"button"} className="AddNewProfile" value="Add new profile"></input>
            <input className="saveProfile" type={"button"} value="Save profile" onClick={() => saveProfile()}>
            </input><p>Save profile</p>
        </div>
    )
}

export default Imputs;
