'use client'

import axios from "axios"
import { PanelWraper } from "../dashboard";
import { PanelContentType } from "../panel/panel";
import { UserQuery } from "../../user-query/model/user-query";
import { ChartType } from "../../visualizations/types";

export interface PanelDto {
    model_id: string,
    panel_id: string,
    user_id: string,
    panel: {
        layout: {
            x: number,
            y: number,
            w: number,
            h: number,
            minH: number,
            minW: number,
        },
        props: {
            name: string,
            content_type: {
                Chart: {
                    chart_type: ChartType
                }
            }
        },
        query: UserQuery["query"];
    }
}

export const postPanel = async (panel: PanelWraper, userId: string = "default") => {
    const body: PanelDto = {
        model_id: panel.props.builder.getModel().id,
        panel_id: panel.props.id,
        user_id: userId,
        panel: {
            layout: {
                x: panel.layout.x,
                y: panel.layout.y,
                w: panel.layout.w,
                h: panel.layout.h,
                minH: panel.layout.minH || 0,
                minW: panel.layout.minW || 0,
            },
            props: {
                name: panel.props.name,
                content_type: {
                    Chart: {
                        chart_type: panel.props.content.chartType || "table"
                    }
                }
            },
            query: panel.props.builder.build().query
        }
    }
    return axios.post('http://127.0.0.1:8000/dashboard/panel/save',
        { ...body }
    );
} 
