import 'regenerator-runtime/runtime';
import React, { useState, useEffect } from 'react';
import './assets/styles/global.css';
//import fontEngine from 'font-engine';
// Components
import SignInPrompt from './components/SignInPrompt';
import SignOutButton from './components/SignOutButton';

export default function App({ isSignedIn, fontFactory, wallet }) {
    const [valueFromBlockchain, setValueFromBlockchain] = useState(),
        [wait, setWait] = useState(true),
        [customFontID, setCustomFontID] = useState('');

    // Get blockchian state once on component load
    useEffect(() => {
        fontFactory
            .getFontid()
            .then(setValueFromBlockchain)
            .catch(alert)
            .finally(() => {
                setWait(false);
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

    const createCustomFont = e => {
        e.preventDefault();
        setWait(true);
        fontFactory
            .createCustomFont(customFontID)
            .then(async () => {
                return fontFactory.getFontid();
            })
            .then(setValueFromBlockchain)
            .finally(() => {
                setWait(false);
                setCustomFontID('');
                /*
                // Package in progress
                fontEngine.mint_font(
                    `${customFontID}${wallet.accountId}`
                );
                */
            });
    };

    const handleInput = e => {
        setCustomFontID(e.target.value);
    };

    return (
        <>
            <main className={wait ? 'wait' : ''}>
                <div className='logo'></div>
                <h1>FontFactory</h1>
                <SignOutButton
                    accountId={wallet.accountId}
                    onClick={() => wallet.signOut()}
                />
                <form onSubmit={createCustomFont} className='new-font'>
                    <h3>Mint Custom Font</h3>
                    <div className='mint-input'>
                        <input
                            style={{ width: '100%' }}
                            autoComplete='off'
                            placeholder='Write your Font ID'
                            value={customFontID}
                            onChange={handleInput}
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
