import { Model, View } from "afast"
import { UserModel } from "../model/user"

export class UserView implements View {
    model: Model = new UserModel()
    allowed: string[] = ['GET', 'POST', 'PUT', 'DELETE']
}
