export default function SignInPrompt({ fontid, onClick }) {
    return (
        <main>
            <div className='logo'></div>
            <h1>FontFactory</h1>
            <p>
                Sign in and mint an instance of Paradisio with an Unique Font
                Identifier
            </p>

            <div className='fontid'>
                <p>Latest Font ID</p>
                <p className='latest'>
                    <code>{fontid}</code>
                </p>
            </div>
            <p>
                <button onClick={onClick}>Sign in with NEAR Wallet</button>
            </p>
        </main>
    );
}
