import 'regenerator-runtime/runtime';
import React, { useState, useEffect } from 'react';
import './assets/styles/global.css';
// Components
import SignInPrompt from './components/SignInPrompt';
import SignOutButton from './components/SignOutButton';

export default function App({ isSignedIn, fontFactory, wallet }) {
    const [valueFromBlockchain, setValueFromBlockchain] = useState(),
        [uiPleaseWait, setUiPleaseWait] = useState(true);

    // Get blockchian state once on component load
    useEffect(() => {
        fontFactory
            .getFontid()
            .then(setValueFromBlockchain)
            .catch(alert)
            .finally(() => {
                setUiPleaseWait(false);
            });
    }, []);

    /// If user not signed-in with wallet - show prompt
    if (!isSignedIn) {
        // Sign-in flow will reload the page later
        return (
            <SignInPrompt
                fontid={valueFromBlockchain}
                onClick={() => wallet.signIn()}
            />
        );
    }

    function createCustomFont(e) {
        e.preventDefault();
        setUiPleaseWait(true);
        const { fontidInput } = e.target.elements;
        fontFactory
            .createCustomFont(fontidInput.value)
            .then(async () => {
                return fontFactory.getFontid();
            })
            .then(setValueFromBlockchain)
            .finally(() => {
                setUiPleaseWait(false);
            });
    }

    return (
        <>
            <main className={uiPleaseWait ? 'please-wait' : ''}>
                <div className='logo'></div>
                <h1>FontFactory</h1>
                <SignOutButton
                    accountId={wallet.accountId}
                    onClick={() => wallet.signOut()}
                />
                <form onSubmit={createCustomFont} className='new-font'>
                    <h3>Mint Custom Font</h3>
                    <div>
                        <input
                            autoComplete='off'
                            defaultValue={valueFromBlockchain}
                            id='fontidInput'
                        />
                        <button>
                            <span>Mint</span>
                            <div className='loader'></div>
                        </button>
                    </div>
                </form>
                <div className='fontid'>
                    <p>Latest Font ID</p>
                    <p className='latest'>
                        <code>{valueFromBlockchain}</code>
                    </p>
                </div>
            </main>
        </>
    );
}
