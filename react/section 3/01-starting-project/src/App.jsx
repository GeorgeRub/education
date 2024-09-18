import {CORE_CONCEPTS, EXAMPLES} from "./data";
import Header from "./components/Header";
import CoreConcepts from "./components/CoreConcepts";
import TabButton from "./components/TabButton";
import {useState} from "react";

function App() {

    const [selectedTopic, setSelectedTopic] = useState();

    let tabContent = <p>Please select a topic</p>;

    if(selectedTopic) {
        tabContent = <div id="tab-content">
            <h3>{EXAMPLES[selectedTopic].title}</h3>
            <p>{EXAMPLES[selectedTopic].description}</p>
            <pre>
                            <code>
                                {EXAMPLES[selectedTopic].code}
                            </code>
                        </pre>
        </div>;
    }

    function handleSelect(someText) {
        setSelectedTopic(someText);
    }

    return (
        <div>
            <Header/>
            <main>
                <section id="core-concepts">
                    <h2>Core Concepts</h2>
                    <ul>
                        <CoreConcepts {...CORE_CONCEPTS[0]} />
                        <CoreConcepts {...CORE_CONCEPTS[1]} />
                        <CoreConcepts {...CORE_CONCEPTS[2]} />
                        <CoreConcepts {...CORE_CONCEPTS[3]} />
                    </ul>
                </section>
                <section id="examples">
                    <h2>Examples</h2>
                    <menu>
                        <TabButton clickEvent={() => handleSelect('components')}>Components</TabButton>
                        <TabButton clickEvent={() => handleSelect('jsx')}>JSX</TabButton>
                        <TabButton clickEvent={() => handleSelect('props')}>Props</TabButton>
                        <TabButton clickEvent={() => handleSelect('state')}>State</TabButton>
                    </menu>
                    {tabContent}
                </section>
            </main>
        </div>
    );
}

export default App;
