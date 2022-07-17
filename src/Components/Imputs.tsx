import React from "react";

const Imputs = () => {
    return (
        <div className="">
            <input type={"text"} placeholder="URL to server"></input>
            <input type={"text"} placeholder="Your secret token"></input>
            <input type={"text"} placeholder="Login"></input>
            <input type={"text"} placeholder="Password"></input>
            <input type={"button"} className="AddNewProfile" value="Add new profile"></input>
            <input type={"checkbox"}>
            </input><p>Save profile</p>
        </div>
    )
}

export default Imputs;
