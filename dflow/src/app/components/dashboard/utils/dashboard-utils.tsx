import { v4 } from "uuid";
import { DataModel } from "../../../model/data-model";
import { UserQueryBuilder } from "../../user-query/model/user-query";
import { ChartType } from "../../visualizations/types";
import { PanelWraper } from "../dashboard";
import { widthInPixels, heightInPixels } from "../dflow-grid";
import { PanelDto } from "../services/post-panel";

export function panelWrapperFactory(
    builder: UserQueryBuilder,
    chartType: ChartType
  ): PanelWraper {
    const id = v4();
    return {
      layout: {
        i: id,
        x: 0,
        y: 0,
        w: 10,
        h: 10,
        minH: 5,
        minW: 5,
        isBounded: false,
      },
      props: {
        builder: builder,
        name: "PanelName: TODO",
        id: id,
        content: {
          type: "Chart",
          chartType: chartType,
        },
        width: widthInPixels(10),
        height: heightInPixels(10),
        resizing: false,
        onContentChange(builder, chartType) {},
      },
    };
  }
  
  export function panelWrapperFromDto(panelDto: PanelDto, model: DataModel) {
    const builder = new UserQueryBuilder(model);
    builder.fromQueryDto(panelDto.panel.query);
    const wrapper: PanelWraper = {
      layout: {
        ...panelDto.panel.layout,
        i: panelDto.panel_id,
        isBounded: false,
      },
      props: {
        content: {
          type: "Chart",
          chartType: panelDto.panel.props.content_type.Chart.chart_type,
        },
        width: widthInPixels(panelDto.panel.layout.w),
        height: heightInPixels(panelDto.panel.layout.h),
        id: panelDto.panel_id,
        name: panelDto.panel.props.name,
        onContentChange(builder, chartType) {},
        resizing: false,
        builder: builder,
      },
    };
  
    return wrapper;
  }
