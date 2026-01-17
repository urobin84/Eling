// Custom event for avatar updates
export const AVATAR_UPDATED_EVENT = 'avatar-updated';

export function emitAvatarUpdate() {
    window.dispatchEvent(new CustomEvent(AVATAR_UPDATED_EVENT));
}

export function onAvatarUpdate(callback: () => void) {
    window.addEventListener(AVATAR_UPDATED_EVENT, callback);
    return () => window.removeEventListener(AVATAR_UPDATED_EVENT, callback);
}
