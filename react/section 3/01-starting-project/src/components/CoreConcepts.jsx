function CoreConcepts({image, title, description}) {
    return(
      <li>
         <img src={image} alt={title} />
        <div>{title}</div>
        <p>{description}</p>
      </li>
    )
  }

  export default CoreConcepts;