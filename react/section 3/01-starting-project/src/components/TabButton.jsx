function TabButton({children, clickEvent, isSelected}) {
    return (
        <li>
            <button className={isSelected ? 'active' : ''} onClick={clickEvent}>{children}</button>
        </li>
    );
}

export default TabButton;
