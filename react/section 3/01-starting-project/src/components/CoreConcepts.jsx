function CoreConcepts(props){
    return(
      <li>
         <img src={props.image} alt={props.title} />
        <div>{props.title}</div>
        <p>{props.description}</p>
      </li>
    )
  }

  export default CoreConcepts;