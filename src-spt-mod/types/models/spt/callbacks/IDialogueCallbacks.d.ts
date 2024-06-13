import { IEmptyRequestData } from '@spt/models/eft/common/IEmptyRequestData';
import { IFriendRequestData } from '@spt/models/eft/dialog/IFriendRequestData';
import { IGetAllAttachmentsRequestData } from '@spt/models/eft/dialog/IGetAllAttachmentsRequestData';
import { IGetAllAttachmentsResponse } from '@spt/models/eft/dialog/IGetAllAttachmentsResponse';
import { IGetChatServerListRequestData } from '@spt/models/eft/dialog/IGetChatServerListRequestData';
import { IGetFriendListDataResponse } from '@spt/models/eft/dialog/IGetFriendListDataResponse';
import { IGetMailDialogInfoRequestData } from '@spt/models/eft/dialog/IGetMailDialogInfoRequestData';
import { IGetMailDialogListRequestData } from '@spt/models/eft/dialog/IGetMailDialogListRequestData';
import { IGetMailDialogViewRequestData } from '@spt/models/eft/dialog/IGetMailDialogViewRequestData';
import { IGetMailDialogViewResponseData } from '@spt/models/eft/dialog/IGetMailDialogViewResponseData';
import { IPinDialogRequestData } from '@spt/models/eft/dialog/IPinDialogRequestData';
import { IRemoveDialogRequestData } from '@spt/models/eft/dialog/IRemoveDialogRequestData';
import { ISendMessageRequest } from '@spt/models/eft/dialog/ISendMessageRequest';
import { ISetDialogReadRequestData } from '@spt/models/eft/dialog/ISetDialogReadRequestData';
import { IGetBodyResponseData } from '@spt/models/eft/httpResponse/IGetBodyResponseData';
import { INullResponseData } from '@spt/models/eft/httpResponse/INullResponseData';
import { DialogueInfo } from '@spt/models/eft/profile/ISptProfile';
export interface IDialogueCallbacks {
  getFriendList(
    url: string,
    info: IEmptyRequestData,
    sessionID: string,
  ): IGetBodyResponseData<IGetFriendListDataResponse>;
  getChatServerList(
    url: string,
    info: IGetChatServerListRequestData,
    sessionID: string,
  ): IGetBodyResponseData<any[]>;
  getMailDialogList(
    url: string,
    info: IGetMailDialogListRequestData,
    sessionID: string,
  ): IGetBodyResponseData<DialogueInfo[]>;
  getMailDialogView(
    url: string,
    info: IGetMailDialogViewRequestData,
    sessionID: string,
  ): IGetBodyResponseData<IGetMailDialogViewResponseData>;
  getMailDialogInfo(
    url: string,
    info: IGetMailDialogInfoRequestData,
    sessionID: string,
  ): IGetBodyResponseData<any>;
  removeDialog(
    url: string,
    info: IRemoveDialogRequestData,
    sessionID: string,
  ): IGetBodyResponseData<any[]>;
  pinDialog(
    url: string,
    info: IPinDialogRequestData,
    sessionID: string,
  ): IGetBodyResponseData<any[]>;
  unpinDialog(
    url: string,
    info: IPinDialogRequestData,
    sessionID: string,
  ): IGetBodyResponseData<any[]>;
  setRead(
    url: string,
    info: ISetDialogReadRequestData,
    sessionID: string,
  ): IGetBodyResponseData<any[]>;
  getAllAttachments(
    url: string,
    info: IGetAllAttachmentsRequestData,
    sessionID: string,
  ): IGetBodyResponseData<IGetAllAttachmentsResponse>;
  listOutbox(url: string, info: IEmptyRequestData, sessionID: string): IGetBodyResponseData<any[]>;
  listInbox(url: string, info: IEmptyRequestData, sessionID: string): IGetBodyResponseData<any[]>;
  sendFriendRequest(url: string, request: IFriendRequestData, sessionID: string): INullResponseData;
  sendMessage(
    url: string,
    request: ISendMessageRequest,
    sessionID: string,
  ): IGetBodyResponseData<number>;
  update(): boolean;
}
