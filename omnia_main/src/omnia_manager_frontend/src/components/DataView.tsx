import ReactJson from "react-json-view";

interface IProps {
  data: any;
}

const DataView: React.FC<IProps> = ({ data }) => {
  return (
    <ReactJson
      src={data}
      collapsed={false}
      displayDataTypes={false}
      displayObjectSize={false}
      name={false}
    />
  );
};

export default DataView;
