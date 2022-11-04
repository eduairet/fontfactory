export default function SignInPrompt({ fontid, onClick }) {
    return (
        <main>
            <h1>FontFactory</h1>
            <p>
                Sign in and mint an instance of Paradisio with an Unique Font
                Identifier
            </p>
            <p>
                Latest Font ID: <code>{fontid}</code>
            </p>
            <p>
                <button onClick={onClick}>Sign in with NEAR Wallet</button>
            </p>
        </main>
    );
}
