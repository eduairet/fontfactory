import { useState } from 'react';

export default function SignOutButton({ accountId, onClick }) {
    const [onButton, setOnButton] = useState(false);
    const handleEnter = () => {
            setOnButton(true);
        },
        handleOut = () => {
            setOnButton(false);
        };
    return (
        <button
            style={{ width: '100%' }}
            onMouseEnter={handleEnter}
            onMouseLeave={handleOut}
            onClick={onClick}
        >
            {onButton ? 'Log out' : accountId}
        </button>
    );
}
